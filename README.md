# Slackup

A quick tool for retrieving all of your slack messages and threads from some channel in the form of JSON.

To use this, you'll need a token, which you can obtain by creating a Slack App. Tokens look something like `xoxp-295847362758-204736406723-8493758473625-2847e94f0b938e9f839b06928be9bf89`. Your token will need the `users:read` scope, as well as scopes depending on which conversations you would like to download:

- `channels:history` and `channels:read` to download from public channels.
- `groups:history` and `groups:read` to download from private groups.
- `im:history` and `im:read` to download direct messages.
- `mpim:history` and `mpim:read` to download from group direct messages.

You'll also need to know the channel ID that you'd like to back up messages from, which you can find by clicking on the channel and then "copy link". The channel ID is the last part of the link and looks something like `FHVT1ABCD`.

With these in hand, you can run `slackup --channel CHANNEL --token TOKEN`, substituting in the actual values that you obtained for the `CHANNEL` and `TOKEN`.

# Working with the output

Using a tool like `jq` (here, v1.6), one can quite easily manipulate this output into different forms.

Given `jq`, the file `jq_stats.sh` can be provided the output from running `slackup` to get some messages, and prints various statistics about the messages.

# Installation

This program was built using `Rust 1.43.0`. You can install Rust from `https://rustup.rs/`.

With Rust installed, run this to compile and install this program:

```
cargo install --git https://github.com/jsdw/slackup
```

# Manually obtaining a user token

If you need to obtain a user token given an app, you can do so manually using `curl` to go through the auth flow

First, make sure that your App is allowed to redirect to `http://localhost:12345`.

Next, browse to this URL, substituting `CLIENT_ID` with your Slack App's client ID:

https://slack.com/oauth/authorize?client_id=CLIENT_ID&scope=users:read%20im:history%20groups:read%20groups:history%20im:read%20channels:read%20channels:history%20mpim:read%20mpim:history&redirect_uri=http://localhost:12345

After clicking to authenticate, you'll be redirected to a page that will look naff (it doesn't exist). But, the URL is all you need. You'll need to copy the value in the query parameters for the "code"; it'll look something like `295847362758.204736406723.2847e94f0b938e9f839b06928be9bf892847e94f0b938e9f839b06928be9bf89`.

With that `CODE` in hand, and with the `CLIENT_ID` and `CLIENT_SECRET` from your Slack App, you can now run this (within 10 minutes of authenticating with the above URL) to get a token for your user:

```
curl https://slack.com/api/oauth.access \
    --data-urlencode 'client_id=CLIENT_ID' \
    --data-urlencode 'client_secret=CLIENT_SECRET' \
    --data-urlencode 'code=CODE' \
    --data-urlencode 'redirect_uri=http://localhost:12345'
```

This will hand back some JSON which contains the token.
