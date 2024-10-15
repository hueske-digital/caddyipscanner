```
# EXTERNAL SERVICE #

https://domain.tld {
    # import logging
    import tls
    import compression
    import header

    @denyallexceptdefined remote_ip 1.1.1.1 8.8.8.8 #IPs with this matcher will be updated

    handle @denyallexceptdefined {
        reverse_proxy service-1:8081
    }
    respond 403
}
```