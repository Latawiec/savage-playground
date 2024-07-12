# savage_playground_frontend

sudo apt install node
sudo apt install npm


## updating node
sudo npm cache clean -f
sudo npm install -g n
sudo n stable

## Project setup
```
npm install --legacy-peer-deps // needed for newer versions of node apparently. I set it for the entire project in npmrc.
```

### Compiles and hot-reloads for development
```
npm run serve
```

### Compiles and minifies for production
```
npm run build
```

### Run your unit tests
```
npm run test:unit
```

### Lints and fixes files
```
npm run lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
