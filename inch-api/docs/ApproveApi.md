# \ApproveApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chain_approve_controller_get_allowance**](ApproveApi.md#chain_approve_controller_get_allowance) | **GET** /v5.0/{chain}/approve/allowance | Get the number of tokens that the 1inch router is allowed to spend
[**chain_approve_controller_get_call_data**](ApproveApi.md#chain_approve_controller_get_call_data) | **GET** /v5.0/{chain}/approve/transaction | Generate data for calling the contract in order to allow the 1inch router to spend funds
[**chain_approve_controller_get_spender**](ApproveApi.md#chain_approve_controller_get_spender) | **GET** /v5.0/{chain}/approve/spender | Address of the 1inch router that must be trusted to spend funds for the exchange



## chain_approve_controller_get_allowance

> chain_approve_controller_get_allowance(chain, token_address, wallet_address)
Get the number of tokens that the 1inch router is allowed to spend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**token_address** | **String** | Token address you want to exchange | [required] |
**wallet_address** | **String** | Wallet address for which you want to check | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chain_approve_controller_get_call_data

> crate::models::ApproveCalldataResponseDto chain_approve_controller_get_call_data(chain, token_address, amount)
Generate data for calling the contract in order to allow the 1inch router to spend funds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**token_address** | **String** | Token address you want to exchange | [required] |
**amount** | Option<**String**> | The number of tokens that the 1inch router is allowed to spend.If not specified, it will be allowed to spend an infinite amount of tokens. |  |

### Return type

[**crate::models::ApproveCalldataResponseDto**](ApproveCalldataResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chain_approve_controller_get_spender

> crate::models::ApproveSpenderResponseDto chain_approve_controller_get_spender(chain)
Address of the 1inch router that must be trusted to spend funds for the exchange

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |

### Return type

[**crate::models::ApproveSpenderResponseDto**](ApproveSpenderResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

