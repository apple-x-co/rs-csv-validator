# rs-csv-validator

## About

JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "id": {
      "type": "integer",
      "minimum": 1
    },
    "name": {
      "type": "string",
      "minLength": 2
    },
    "yes": {
      "type": "boolean"
    }
  },
  "required": [
    "id",
    "name",
    "yes"
  ]
}
```

CSV

```text
id,name,yes
1,tom,true
2,miki,false
```

Validation

```bash
rs-csv-validator --csv work/test.csv --schema work/test.json
```