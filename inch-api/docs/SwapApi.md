# \SwapApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**exchange_controller_get_quote**](SwapApi.md#exchange_controller_get_quote) | **GET** /v5.0/{chain}/quote | Find the best quote to exchange via 1inch router
[**exchange_controller_get_swap**](SwapApi.md#exchange_controller_get_swap) | **GET** /v5.0/{chain}/swap | Generate data for calling the 1inch router for exchange



## exchange_controller_get_quote

> crate::models::QuoteResponseDto exchange_controller_get_quote(chain, from_token_address, to_token_address, amount, protocols, fee, gas_limit, connector_tokens, complexity_level, main_route_parts, parts, gas_price)
Find the best quote to exchange via 1inch router

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**from_token_address** | **String** |  | [required] |
**to_token_address** | **String** |  | [required] |
**amount** | **String** |  | [required] |
**protocols** | Option<**String**> | default: all |  |
**fee** | Option<**String**> | Min: 0; max: 3; Max: 0; max: 3; default: 0;  !should be the same for quote and swap! |  |
**gas_limit** | Option<**f32**> |  |  |
**connector_tokens** | Option<[**serde_json::Value**](.md)> | max: 5; !should be the same for quote and swap! |  |
**complexity_level** | Option<**f32**> | min: 0; max: 3; default: 2; !should be the same for quote and swap! |  |
**main_route_parts** | Option<[**serde_json::Value**](.md)> | default: 10; max: 50  !should be the same for quote and swap! |  |
**parts** | Option<[**serde_json::Value**](.md)> | split parts. default: 50;  max: 100!should be the same for quote and swap! |  |
**gas_price** | Option<**String**> | default: fast from network |  |

### Return type

[**crate::models::QuoteResponseDto**](QuoteResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exchange_controller_get_swap

> crate::models::SwapResponseDto exchange_controller_get_swap(chain, from_token_address, to_token_address, amount, from_address, slippage, protocols, dest_receiver, referrer_address, fee, disable_estimate, permit, compatibility_mode, burn_chi, allow_partial_fill, parts, main_route_parts, connector_tokens, complexity_level, gas_limit, gas_price)
Generate data for calling the 1inch router for exchange

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**from_token_address** | **String** |  | [required] |
**to_token_address** | **String** |  | [required] |
**amount** | **String** |  | [required] |
**from_address** | **String** | The address that calls the 1inch contract | [required] |
**slippage** | **f32** | min: 0; max: 50; | [required] |
**protocols** | Option<**String**> | default: all |  |
**dest_receiver** | Option<**String**> | Receiver of destination currency. default: fromAddress |  |
**referrer_address** | Option<**String**> |  |  |
**fee** | Option<**String**> | Min: 0; max: 3; Max: 0; max: 3; default: 0;  !should be the same for quote and swap! |  |
**disable_estimate** | Option<**bool**> |  |  |
**permit** | Option<**String**> | https://eips.ethereum.org/EIPS/eip-2612 |  |
**compatibility_mode** | Option<**bool**> | Allows to build calldata without optimized routers |  |
**burn_chi** | Option<**bool**> | default: false; Suggest to check user's balance and allowance before set this flag; CHI should be approved to spender address |  |
**allow_partial_fill** | Option<**bool**> |  |  |
**parts** | Option<[**serde_json::Value**](.md)> | split parts. default: 50;  max: 100!should be the same for quote and swap! |  |
**main_route_parts** | Option<[**serde_json::Value**](.md)> | default: 10; max: 50  !should be the same for quote and swap! |  |
**connector_tokens** | Option<[**serde_json::Value**](.md)> | max: 5; !should be the same for quote and swap! |  |
**complexity_level** | Option<**f32**> | min: 0; max: 3; default: 2; !should be the same for quote and swap! |  |
**gas_limit** | Option<**String**> |  |  |
**gas_price** | Option<**String**> | default: fast from network |  |

### Return type

[**crate::models::SwapResponseDto**](SwapResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

