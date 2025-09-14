# Writing a custom Config File

Configuration files for Prometheus are JSON documents that describe the desired settings. Let's say we have the following config file:

{% code title="config.json" %}
```json
{
    "LuaVersion": "Lua51",
    "VarNamePrefix": "",
    "NameGenerator": "MangledShuffled",
    "PrettyPrint": false,
    "Seed": 0,
    "Steps": [
        {
            "Name": "ConstantArray",
            "Settings": {
                "StringsOnly": true,
                "Treshold": 1
            }
        }
    ]
}
```
{% endcode %}

One can now obfuscate a script using this configuration by running:

```sh
cargo run --bin prometheus -- --config config.json hello_world.lua
```
