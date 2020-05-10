# Slackup

A quick tool for retrieving all of your slack messages and threads from some channel in the form of JSON.

To use this, you'll need a token, which you can obtain by creating a Slack App. Tokens look something like `xoxp-295847362758-204736406723-8493758473625-2847e94f0b938e9f839b06928be9bf89`.

You'll also need to know the channel ID that you'd like to back up messages from, which you can find by clicking on the channel and then "copy link". The channel ID is the last part of the link and looks something like `FHVT1ABCD`.

With these in hand, you can run `slackup --channel CHANNEL --token TOKEN`, substituting in the actual values that you obtained for the `CHANNEL` and `TOKEN`.

