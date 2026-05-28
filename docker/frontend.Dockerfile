FROM node:24-alpine AS builder

ARG VITE_API_BASE_URL=/api
ENV VITE_API_BASE_URL=${VITE_API_BASE_URL}

WORKDIR /app/frontend

COPY frontend/package.json ./
COPY frontend/pnpm-lock.yaml* ./
COPY frontend/package-lock.json* ./
COPY frontend/yarn.lock* ./

RUN corepack enable \
    && if [ -f pnpm-lock.yaml ]; then pnpm install --frozen-lockfile; \
       elif [ -f package-lock.json ]; then npm ci; \
       elif [ -f yarn.lock ]; then yarn install --frozen-lockfile; \
       else npm install; fi

COPY frontend ./

RUN if [ -f pnpm-lock.yaml ]; then pnpm build; \
    elif [ -f package-lock.json ]; then npm run build; \
    elif [ -f yarn.lock ]; then yarn build; \
    else npm run build; fi

FROM nginx:1.29-alpine

COPY docker/nginx/default.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/frontend/dist /usr/share/nginx/html

EXPOSE 80
