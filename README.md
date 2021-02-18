# ddns
A dynamic DNS client, use it serve on your machine if you no public ip address.

# Quick start
```
ddns -conf ddns.conf
```
Edit ddns.conf typing your config:
```json
{
  "check_seconds": 60,
  "dns_config": [
    {
      "dns_sp": "dnspod",
      "api_id": "73840",
      "api_token": "c45f9a093c15daf7c74bfb9bdccace10",
      "domains": [
        {
          "domain": "to2.net",
          "records": [
            {
              "name": "@",
              "ttl":60,
              "dyn_pub": 0
            }
          ]
        }
      ]
    }
  ]
}
```