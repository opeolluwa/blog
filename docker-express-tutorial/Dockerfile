# state the base image/ node versionb
FROM node:20-alpine 

# specify the worj directory 
WORKDIR /app

COPY . .

# install deps 
RUN npm install --save

# expose the port
EXPOSE 3000

# start the app
CMD ["npm", "start"]

# docker build -t node-express-app .