```
# EXTERNAL SERVICE #

https://domain.tld {
    # import logging
    import tls
    import compression
    import header

@denyallexceptdefined not remote_ip 1.1.1.1 8.8.8.8 # IPs with this matcher will be updated
    abort @denyallexceptdefined

    respond "Up and running"
}
```