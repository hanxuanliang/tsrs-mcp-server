#![allow(dead_code)]

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitInt, Type};

/// Options for the TsEndpoint derive macro
#[derive(Debug, FromMeta)]
struct EndpointOpts {
    /// API name for the Tushare request
    api: String,
    /// Description of the API endpoint
    desc: String,
    /// Response type (optional)
    #[darling(default)]
    resp: Option<syn::Path>,
}

/// Options for the TsResponse derive macro
#[derive(Debug, FromMeta)]
struct ResponseOpts {
    /// API name for the Tushare response
    api: String,
}

/// Derive macro for Tushare API endpoints
///
/// Example usage:
/// ```rust
/// #[derive(TsEndpoint)]
/// #[endpoint(api = "api_name", desc = "description", resp = MyResponseType)]
/// struct MyRequest {
///     // ... fields ...
/// }
/// ```
#[proc_macro_derive(TsEndpoint, attributes(endpoint, fields))]
pub fn ts_endpoint_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let requester_name = syn::Ident::new(&format!("{}Requester", name), name.span());

    // Parse endpoint options using darling
    let endpoint_opts = match input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("endpoint"))
        .map(|attr| EndpointOpts::from_meta(&attr.meta))
        .transpose()
    {
        Ok(Some(opts)) => opts,
        Ok(None) => {
            return syn::Error::new_spanned(
                input.ident.clone(),
                "Missing #[endpoint(...)] attribute",
            )
            .to_compile_error()
            .into()
        }
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    // Extract fields for request parameters
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(
                    input.ident.clone(),
                    "TsEndpoint only supports structs with named fields",
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new_spanned(input.ident.clone(), "TsEndpoint only supports structs")
                .to_compile_error()
                .into()
        }
    };

    // Generate field serialization for the params object
    let param_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();

        // Check for serde rename attribute
        let mut rename_value = None;
        for attr in &field.attrs {
            if attr.path().is_ident("serde") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("rename") {
                        rename_value = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                    }
                    Ok(())
                });
            }
        }

        // Use rename value if present, otherwise use field name
        let param_name = rename_value.unwrap_or_else(|| field_name_str.clone());

        quote! {
            params.insert(#param_name.to_string(), serde_json::to_value(&self.#field_name)?);
        }
    });

    // Get API name and description from the endpoint options
    let api_name = &endpoint_opts.api;
    let api_desc = &endpoint_opts.desc;

    // Check if response type is specified
    let resp_type = endpoint_opts.resp.as_ref().map(|path| quote! { #path });

    // Generate the TsRequesterImpl struct implementation with a unique name
    let ts_requester_impl = if let Some(resp_type) = resp_type.clone() {
        quote! {
            // 定义单独的TsRequester结构体和impl，这个结构体是在当前crate中的
            pub struct #requester_name {
                request: #name,
                fields: Option<Vec<&'static str>>,
            }

            impl #requester_name {
                pub fn new(request: #name, fields: Option<Vec<&'static str>>) -> Self {
                    Self { request, fields }
                }

                pub fn with_fields(mut self, fields: Vec<&'static str>) -> Self {
                    self.fields = Some(fields);
                    self
                }

                pub async fn execute(self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
                    self.request.__execute_request(self.fields).await
                }

                pub async fn execute_typed(self) -> Result<Vec<#resp_type>, Box<dyn std::error::Error>> {
                    // If fields are not provided, extract field names from the response struct
                    let fields_to_use = if self.fields.is_none() {
                        // Get field names from the response struct by reflection
                        let field_names = <#resp_type>::get_field_names();
                        Some(field_names)
                    } else {
                        self.fields
                    };

                    // Execute with the fields (either provided or derived)
                    let json = self.request.__execute_request(fields_to_use).await?;
                    let res = <#resp_type>::from_json(&json);
                    res
                }

                pub async fn execute_as_dicts(self) -> Result<Vec<std::collections::HashMap<String, serde_json::Value>>, Box<dyn std::error::Error>> {
                    use serde_json::Value;
                    use std::collections::HashMap;

                    // 直接使用__execute_request而不是execute，以便保留字段信息
                    let json = self.request.__execute_request(self.fields).await?;

                    // Extract fields and items
                    let data = json.get("data")
                        .ok_or("Missing 'data' field in response")?;

                    let fields = data.get("fields")
                        .ok_or("Missing 'fields' field in data")?
                        .as_array()
                        .ok_or("'fields' is not an array")?;

                    let items = data.get("items")
                        .ok_or("Missing 'items' field in data")?
                        .as_array()
                        .ok_or("'items' is not an array")?;

                    // Convert to Vec<HashMap<String, Value>>
                    let mut result = Vec::with_capacity(items.len());

                    for item_value in items {
                        let item = item_value.as_array()
                            .ok_or("Item is not an array")?;

                        let mut map = HashMap::new();

                        // Map fields to values
                        for (i, field) in fields.iter().enumerate() {
                            if i < item.len() {
                                let field_name = field.as_str()
                                    .ok_or("Field name is not a string")?
                                    .to_string();

                                map.insert(field_name, item[i].clone());
                            }
                        }

                        result.push(map);
                    }

                    Ok(result)
                }
            }
        }
    } else {
        quote! {
            // 定义单独的TsRequester结构体和impl，这个结构体是在当前crate中的
            pub struct #requester_name {
                request: #name,
                fields: Option<Vec<&'static str>>,
            }

            impl #requester_name {
                pub fn new(request: #name, fields: Option<Vec<&'static str>>) -> Self {
                    Self { request, fields }
                }

                pub fn with_fields(mut self, fields: Vec<&'static str>) -> Self {
                    self.fields = Some(fields);
                    self
                }

                pub async fn execute(self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
                    self.request.__execute_request(self.fields).await
                }

                pub async fn execute_as_dicts(self) -> Result<Vec<std::collections::HashMap<String, serde_json::Value>>, Box<dyn std::error::Error>> {
                    use serde_json::Value;
                    use std::collections::HashMap;

                    // 直接使用__execute_request而不是execute，以便保留字段信息
                    let json = self.request.__execute_request(self.fields).await?;

                    // Extract fields and items
                    let data = json.get("data")
                        .ok_or("Missing 'data' field in response")?;

                    let fields = data.get("fields")
                        .ok_or("Missing 'fields' field in data")?
                        .as_array()
                        .ok_or("'fields' is not an array")?;

                    let items = data.get("items")
                        .ok_or("Missing 'items' field in data")?
                        .as_array()
                        .ok_or("'items' is not an array")?;

                    // Convert to Vec<HashMap<String, Value>>
                    let mut result = Vec::with_capacity(items.len());

                    for item_value in items {
                        let item = item_value.as_array()
                            .ok_or("Item is not an array")?;

                        let mut map = HashMap::new();

                        // Map fields to values
                        for (i, field) in fields.iter().enumerate() {
                            if i < item.len() {
                                let field_name = field.as_str()
                                    .ok_or("Field name is not a string")?
                                    .to_string();

                                map.insert(field_name, item[i].clone());
                            }
                        }

                        result.push(map);
                    }

                    Ok(result)
                }
            }
        }
    };

    // Generate impl for the struct
    let impl_struct = quote! {
        impl #name {
            /// Get the API name
            pub fn api_name(&self) -> &'static str {
                #api_name
            }

            /// Get the API description
            pub fn description(&self) -> &'static str {
                #api_desc
            }

            /// Start chain with fields
            pub fn with_fields(self, fields: Vec<&'static str>) -> #requester_name {
                #requester_name::new(self, Some(fields))
            }

            /// Execute without fields
            pub async fn execute(self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
                self.__execute_request(None).await
            }

            /// Execute with typed response, automatically deriving fields from response struct
            pub async fn execute_typed(self) -> Result<Vec<#resp_type>, Box<dyn std::error::Error>> {
                // Create requester and call its execute_typed method
                let requester = #requester_name::new(self, None);
                requester.execute_typed().await
            }

            // Inner method used by TsRequester
            #[doc(hidden)]
            pub(crate) async fn __execute_request(&self, fields: Option<Vec<&str>>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
                use serde_json::{json, Map, Value};
                use reqwest::Client;
                use dotenvy::dotenv;
                use std::env;

                // Load environment variables
                dotenv().ok();

                // Get token from environment
                let token = env::var("TUSHARE_TOKEN")
                    .map_err(|_| "TUSHARE_TOKEN environment variable not set")?;

                // Build params object
                let mut params = Map::new();
                #(#param_fields)*

                // Create request body
                let mut request_body = Map::new();
                request_body.insert("api_name".to_string(), Value::String(#api_name.to_string()));
                request_body.insert("token".to_string(), Value::String(token));
                request_body.insert("params".to_string(), Value::Object(params));

                // Add fields if provided
                if let Some(field_list) = fields {
                    request_body.insert("fields".to_string(),
                        Value::String(field_list.join(",")));
                }

                // Send request
                let client = Client::new();
                let response = client
                    .post("http://api.tushare.pro/")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&Value::Object(request_body))?)
                    .send()
                    .await?;

                if !response.status().is_success() {
                    return Err(format!("Request failed with status: {}", response.status()).into());
                }

                let json = response.json::<Value>().await?;
                Ok(json)
            }
        }
    };

    // Combine implementations
    let output = quote! {
        #impl_struct
        #ts_requester_impl
    };

    output.into()
}

/// Derive macro for Tushare API response models
///
/// This macro will create structs that represent the response data from a Tushare API call.
/// It automatically maps the fields to the data items in the response.
///
/// Example usage:
/// ```rust
/// #[derive(TsResponse)]
/// #[response(api = "api_name")]
/// struct MyResponseData {
///     #[ts_field(0)]
///     field_one: String,
///     #[ts_field(1)]
///     field_two: i64,
///     // ...
/// }
/// ```
#[proc_macro_derive(TsResponse, attributes(response, ts_field))]
pub fn ts_response_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Parse response options
    let response_opts = match input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("response"))
        .map(|attr| ResponseOpts::from_meta(&attr.meta))
        .transpose()
    {
        Ok(Some(opts)) => opts,
        Ok(None) => {
            return syn::Error::new_spanned(
                input.ident.clone(),
                "Missing #[response(...)] attribute",
            )
            .to_compile_error()
            .into()
        }
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    // Extract fields for response data
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(
                    input.ident.clone(),
                    "TsResponse only supports structs with named fields",
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new_spanned(input.ident.clone(), "TsResponse only supports structs")
                .to_compile_error()
                .into()
        }
    };

    // Generate field parsing for the response items
    let field_parsers = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        // Extract index from ts_field attribute
        let mut field_index = None;
        // Check for #[serde(default)] attribute
        let mut has_serde_default = false;

        for attr in &field.attrs {
            if attr.path().is_ident("ts_field") {
                match attr.meta.require_list() {
                    Ok(nested) => {
                        // Parse the first token in the list as a literal integer
                        let lit: LitInt = match syn::parse2(nested.tokens.clone()) {
                            Ok(lit) => lit,
                            Err(e) => return e.to_compile_error(),
                        };
                        field_index = Some(lit.base10_parse::<usize>().unwrap());
                    }
                    Err(e) => return e.to_compile_error(),
                }
            } else if attr.path().is_ident("serde") {
                 // Use parse_nested_meta for a more robust check
                 let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("default") {
                        has_serde_default = true;
                    }
                    // Ignore other serde attributes like rename, skip, etc.
                    // Need to handle potential errors within meta if attributes are complex
                    Ok(())
                });
                // Ignoring potential parse_nested_meta error for simplicity for now
            }
        }

        let index = match field_index {
            Some(idx) => idx,
            None => {
                return syn::Error::new_spanned(field_name, "Missing #[ts_field(index)] attribute")
                    .to_compile_error()
            }
        };

        let from_value = if field_type_is_option(field_type) {
            // Logic for Option<T>
            quote! {
                let #field_name = if item.len() > #index {
                    let val = &item[#index];
                    if val.is_null() {
                        None
                    } else {
                        Some(serde_json::from_value(val.clone())?)
                    }
                } else {
                    None // Treat missing index as None for Option types
                };
            }
        } else if has_serde_default {
            // Logic for non-Option<T> with #[serde(default)]
             quote! {
                let #field_name:#field_type = if item.len() > #index {
                    let val = &item[#index];
                    if val.is_null() {
                        Default::default() // Use default if null
                    } else {
                        // Using unwrap_or_default() on the Result is cleaner
                        serde_json::from_value(val.clone()).unwrap_or_default()
                    }
                } else {
                    Default::default() // Use default if index out of bounds
                };
            }
        } else {
            // Logic for non-Option<T> *without* #[serde(default)]
            quote! {
                let #field_name = if item.len() > #index {
                    let val = &item[#index];
                     // Error on null for non-optional, non-default fields
                    if val.is_null() {
                         return Err(format!("Field '{}' at index {} is null, but type is not Option and #[serde(default)] is not specified", stringify!(#field_name), #index).into());
                    }
                    serde_json::from_value(val.clone())?
                } else {
                    return Err(format!("Field index {} out of bounds for required field '{}'", #index, stringify!(#field_name)).into());
                };
            }
        };

        quote! { #from_value }
    });

    // 生成字段名称列表（用于构造和获取字段名）
    let field_names: Vec<_> = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().clone())
        .collect();

    // 生成用于构造结构体的字段列表
    let struct_field_tokens = {
        let field_idents = &field_names;
        quote! {
            #(#field_idents),*
        }
    };

    // Get API name
    let api_name = &response_opts.api;

    // Generate implementation for parsing response
    let output = quote! {
        impl #name {
            /// Parse a list of items from Tushare API response
            pub fn from_json(json: &serde_json::Value) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
                use serde_json::Value;

                // Extract data from response
                let data = json.get("data")
                    .ok_or_else(|| "Missing 'data' field in response")?;

                let items = data.get("items")
                    .ok_or_else(|| "Missing 'items' field in data")?
                    .as_array()
                    .ok_or_else(|| "'items' is not an array")?;

                let mut result = Vec::with_capacity(items.len());

                for item_value in items {
                    let item = item_value.as_array()
                        .ok_or_else(|| "Item is not an array")?;

                    #(#field_parsers)*

                    result.push(Self {
                        #struct_field_tokens
                    });
                }

                Ok(result)
            }

            /// Get the API name for this response
            pub fn api_name() -> &'static str {
                #api_name
            }

            /// Get field names from the response struct
            pub fn get_field_names() -> Vec<&'static str> {
                vec![
                    #(stringify!(#field_names)),*
                ]
            }
        }

        // Implement From<Value> to allow automatic conversion from JSON
        impl From<serde_json::Value> for #name {
            fn from(value: serde_json::Value) -> Self {
                // This is just a placeholder implementation to satisfy the trait bound
                // The actual conversion is handled by the from_json method
                panic!("Direct conversion from Value to {} is not supported, use from_json instead", stringify!(#name));
            }
        }
    };

    output.into()
}

/// Check if a type is an Option<T>
fn field_type_is_option(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            return segment.ident == "Option";
        }
    }
    false
}
