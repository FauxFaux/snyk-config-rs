# snyk-config

This is a port of [@snyk/config](https://github.com/snyk/config),
an opinionated `npm` library which loads config from files and
the environment.

It prefers:

 * environment variables, prefixed with `CONF_`, as literals or JSON
 * `${CONFIG_SECRET_FILE}`
 * `config.${SERVICE_ENV}.json`
 * `config.default.json`


 * `CONFIG_SECRET_FILE` defaults to `./config.secret.json`
 * `SERVICE_ENV` defaults to `local`

Loaded values are merged, e.g.

`config.default.json`:
```json
{"buy": {"potatoes":  5}}
```

`config.secret.json`:
```json
{"buy": {"condamns":  1}}
```

env:
```bash
export CONF_buy__condamns=7
export CONF_debug=true
```

..will result in:
```json
{"buy": {"condamns": 7, "potatoes":  5}, "debug": true}
```
