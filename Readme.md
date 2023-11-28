# Planned structure

GET `/:uri` follows shortened uri

---

POST `/_shorten` shortens given uri

Needs Data:

```json
{
    "r_uri": "https://example.com"
}
```

---

DELETE `/_delete/:uri` deletes shortened uri

---

GET `/_info/:uri` info about shortened uri

Returns Data:

```json
[
    {
        "client_ip": "192.168.X.X:XXXX",
        "s_uri": "1",
        "timestamp": "2023-11-28 19:46:36"
    },
    {
        "client_ip": "127.0.0.1:53490",
        "s_uri": "3",
        "timestamp": "2023-11-28 20:12:19"
    }
]
```

---