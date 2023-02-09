FROM alpine:3 as swagger

ENV SWAGGER_UI_VERSION 4.15.5

RUN wget "https://github.com/swagger-api/swagger-ui/archive/refs/tags/v${SWAGGER_UI_VERSION}.tar.gz" -O swagger-ui.tar.gz
RUN tar -zxf swagger-ui.tar.gz -C /
RUN mv /swagger-ui-${SWAGGER_UI_VERSION}/dist /swagger-ui

#FROM razum2um/nginx-purge:1.23.3-alpine
FROM openresty/openresty:1.21.4.1-0-bullseye-fat

COPY --from=swagger --chown=nginx:nginx /swagger-ui /usr/share/nginx/html/swagger-ui
RUN sed -i 's|https://petstore.swagger.io/v2/swagger.json|/swagger.json|' /usr/share/nginx/html/swagger-ui/swagger-initializer.js
COPY swagger.json /usr/share/nginx/html/swagger.json

COPY proxy.conf /etc/nginx/proxy.conf
COPY tuning.conf /etc/nginx/conf.d/00_tuning.conf
COPY default.conf /etc/nginx/conf.d/default.conf
