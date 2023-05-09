# \InfoApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chain_presets_controller_get_presets**](InfoApi.md#chain_presets_controller_get_presets) | **GET** /v5.0/{chain}/presets | List of preset configurations for the 1inch router
[**chain_protocols_controller_get_protocols_images**](InfoApi.md#chain_protocols_controller_get_protocols_images) | **GET** /v5.0/{chain}/liquidity-sources | List of liquidity sources that are available for routing in the 1inch Aggregation protocol
[**chain_tokens_controller_get_tokens**](InfoApi.md#chain_tokens_controller_get_tokens) | **GET** /v5.0/{chain}/tokens | List of tokens that are available for swap in the 1inch Aggregation protocol



## chain_presets_controller_get_presets

> chain_presets_controller_get_presets(chain)
List of preset configurations for the 1inch router

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chain_protocols_controller_get_protocols_images

> crate::models::ProtocolsResponseDto chain_protocols_controller_get_protocols_images(chain)
List of liquidity sources that are available for routing in the 1inch Aggregation protocol

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |

### Return type

[**crate::models::ProtocolsResponseDto**](ProtocolsResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chain_tokens_controller_get_tokens

> crate::models::TokensResponseDto chain_tokens_controller_get_tokens(chain)
List of tokens that are available for swap in the 1inch Aggregation protocol

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |

### Return type

[**crate::models::TokensResponseDto**](TokensResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

