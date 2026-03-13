FROM node:20 AS build
WORKDIR /app
COPY web ./web
WORKDIR /app/web

RUN npm ci

# Kill any stray CommonJS vite config that breaks ESM mode
RUN rm -f vite.config.js vite.config.js.*

RUN npm run build

FROM nginx:alpine
COPY --from=build /app/web/dist /usr/share/nginx/html
EXPOSE 80
