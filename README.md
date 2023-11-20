# Rusty Slack 
## Making Private Slack Channels on Demand

This is a work in progress project.

## Setup
Create a slack app
In `Bot Token Scopes` add `groups:write`

## Environment Variables

The project uses the following environment variables:

```
SLACK_BOT_TOKEN=xoxb-xxxxxxxxxxxx-xxxxxxxxxxxxxxxxxxxxxxxx

# Slack Members with their Slack IDs separated by commas
SECURITY=UXXXXXXX1,UXXXXXXX2,UXXXXXXX3
LEGAL=UXXXXXX1X,UXXXXXX2X,UXXXXXX3X
```

## Development (Kinda Ordered)
- [x] Web Axum
- [x] Dockerization
- [ ] Github Actions to Update Docker Image
- [ ] Docker Documentations
- [ ] Make Terminal Tool
