# Objective
Map the configuration of a cloudflare domain to your running network's public IP address.

# Requirements
This code expects a `.cloudflare-ddns.(yaml|yml|json|toml)` file within the running
directory of the code.

# Example Configuration
The following is an example configuration,

```yaml
zone: colinbruner.com                             # This is the name of the zone for DDNS
zone_id: fakj9139r190uflsdkjflsf90kld14           # This is the ID of the CloudClare 'zone'
token: Pk_jkas93jfkkddkjfsdjflsdladfiajslkfjlj    # This is the API token created for your account
```

# Running the code
Ensure your configuration file is in place, and then execute the binary. An enforcing run should
look like the following.

``` bash
❯ ./cloudflare-ddns
IP: 109.24.238.187 == A Record 109.24.238.187. Doing nothing.
```

# Scheduling runs

1. Install the binary on your remote system in a directory
2. Create your `.cloudflare-ddns.(yaml|yml|json|toml)` within the same directoy as the binary.
3. Add the following cronjob.

```
*/15 * * * * cd /etc/<YOUR_DIR> && ./cloudflare-ddns >> /var/log/cloudflare-ddns.log
```

