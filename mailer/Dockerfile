# Basics
FROM node:19.2-alpine
WORKDIR /app

# Update the container
RUN apk upgrade --no-cache --update
RUN apk add --no-cache tzdata

# Configure the user
RUN chown -R node /app
USER node

# Install the packages
COPY --chown=node ./package.json .
RUN yarn install --production

# Copy the project
COPY --chown=node ./tsconfig.json .
COPY --chown=node ./src ./src

# Run the project
RUN yarn run build
CMD yarn run start:prod
