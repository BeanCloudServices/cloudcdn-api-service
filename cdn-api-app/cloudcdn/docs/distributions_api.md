# distributions_api

All URIs are relative to *https://api.bcs.dev.peterbean.net/cloudcdn/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
**createDistribution**](distributions_api.md#createDistribution) | **POST** /distributions | Create new distribution
**getDistribution**](distributions_api.md#getDistribution) | **GET** /students/{id} | Create new distribution


# **createDistribution**
> models::DistributionUpsert createDistribution(distribution_upsert_request)
Create new distribution

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **distribution_upsert_request** | [**DistributionUpsertRequest**](DistributionUpsertRequest.md)| Update an existent distribution | 

### Return type

[**models::DistributionUpsert**](DistributionUpsert.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getDistribution**
> models::DistributionUpsert getDistribution(id)
Create new distribution

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | [****](.md)| ID of student to update | 

### Return type

[**models::DistributionUpsert**](DistributionUpsert.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

