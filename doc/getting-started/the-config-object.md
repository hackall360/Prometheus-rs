# The Config Object

Prometheus takes a configuration object. In this object there can be many properties applied. The following table provides an overview:

| Property      | type    | possible values                              | default           |
| ------------- | ------- | -------------------------------------------- | ----------------- |
| LuaVersion    | string  | "Lua51", "LuaU"                              | "Lua51"           |
| PrettyPrint   | boolean | true, false                                  | false             |
| VarNamePrefix | string  | any                                          | ""                |
| NameGenerator | string  | "Mangled", "MangledShuffled", "Il", "Number" | "MangledShuffled" |
| Seed          | number  | any                                          | 0                 |
| Steps         | array   | Step[]                                       | []                |

As this table shows, all properties in the config object are optional as they have a default value.

As an example, here is the JSON for the minify preset:

```json
{
    "LuaVersion": "Lua51",
    "VarNamePrefix": "",
    "NameGenerator": "MangledShuffled",
    "PrettyPrint": false,
    "Seed": 0,
    "Steps": []
}
```

### Steps

The most important property is the Steps property. This property must be an array of step configurations. A step in Prometheus describes a single transformation applied to your script by the obfuscation pipeline. A step configuration consists of the name of the step as well as settings for the step. All steps will later be applied in the order they are defined. A single step can be defined twice and will then be applied twice.
