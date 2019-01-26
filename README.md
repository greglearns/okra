# Rust API client for Okta

Allows customers to easily access the Okta API

## Overview
This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.9.0
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen
For more information, please visit [http://developer.okta.com/](http://developer.okta.com/)

## Documentation for API Endpoints

See [Docs](docs)

## Rebuilding from OpenAPI

1. Build the client from OpenAPI specs
```
docker run --rm -v $(pwd):$(pwd) -w $(pwd) openapitools/openapi-generator-cli generate \
    --input-spec api.yaml \
    --generator-name rust \
    --output out
```
2. Copy `out/src` over `src` and `out/docs` over `docs`
3. Replace all `::models` with `crate::models` and `::apis` with `crate::apis`
4. Replace all `__` with `_`
5. Replace all `supercrate` with `crate`
6. Add some constructors to `apis::configuration::Configuration`
7. Lots of changes for hyper v0.12
8. Create enums
