# rocket-kandidm test

This is a short guide on how to setup a `kanidm` test environment for testing it
with `rocket`.

## Kanidm setup

This setup is based on the information given at [https://kanidm.github.io](https://kanidm.github.io/kanidm/master/evaluation_quickstart.html).
It is required that you run `kanidmd` as root.

### Server

```bash
# Login as `root`
sudo -i
# Switch to kandidm-test folder
cd kanidm-test
# Generate required certificates
kanidmd cert-generate -c server.toml
# Create required unix socket folder (should be created automatically by `kanidm` but it currently does not)
mkdir /run/kanidmd
# Start server
kanidmd server -c server.toml
# Recover admin account password
kanidmd recover-account admin
# Recover idm_admin account password
kanidmd recover-account idm_admin
```

### Client

Create the following file:
```
# ~/.config/kanidm

uri = "https://localhost:8443"
verify_ca = false
```

Login:
```
kanidm login --name idm_admin
```

Create a test account:
```
kanidm person create testaccount "Test Account"
```

Setup account credentials:
```
kanidm person credential create-reset-token testaccount
```

Use provided reset token to add a password. You can now login using the web interface.

### Setup account and oauth2

```
# Create a test group for the rocket oauth2 test application:
kanidm group create rocket_oauth --name idm_admin
# Add your testaccount
kanidm group add-members rocket_oauth testaccount --name idm_admin
```

```
# Create oauth2 application
kanidm system oauth2 create rocket_testapp "Rocket Oauth2" http://localhost:8000
# Configure redirect URL
kanidm system oauth2 add-redirect-url rocket_testapp http://localhost:8000/auth/kanidm
# Update scope map
kanidm system oauth2 update-scope-map rocket_testapp rocket_oauth email profile openid
# Check what you did
kanidm system oauth2 get rocket_testapp
# Disable PKCE for testing
kanidm system oauth2 warning-insecure-client-disable-pkce rocket_testapp
```
