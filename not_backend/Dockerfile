FROM hayd/deno as build
WORKDIR /app
COPY src/* .
RUN deno bundle ./main.ts

FROM hayd/deno as runtime
WORKDIR /app
COPY --from=build /app/main.bundle.js ./bundle.js
COPY ./static .
CMD deno --allow-env --allow-net=db main.bundle.js