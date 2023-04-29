FROM node:latest

# Install Truffle
RUN npm install -g truffle

# Set the working directory to /app
WORKDIR /app/blockchain

# Install dependencies
COPY package*.json ./
RUN npm install

CMD ["truffle", "console", "--network", "development"]
