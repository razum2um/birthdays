# Birthday app caching

This proxy layer serves several purposes:

- it leverages `srcache-nginx-module` with the app exposing proper `Cache-Control` and `Expires` headers to improve scalability and performance, cache invalidates during `PUT` request
- it hides internal urls like `/metrics` and `/health` from the outer world
- it serves the Swagger UI json definition

Besides, at this layer the standard request throtling could be confgured.
This is not enabled by default since no particular traffic estimation provided in task and not to disturb load testing
