# Discord HALAMADRID Bot Documentation
## Overview
The Discord Football Bot is designed to enhance the experience of football fans on Discord by providing interactive commands that engage users with football-related content. This bot can send messages, share videos, and create a fun, themed environment for football enthusiasts.
## Features
1. `!uno` Command

Description: Sends a sequence of Madrid theme followed by a  vincius video.
<br>
Messages Sent:
<br>
"dos"
<br>
"tres"
<br>
Video: Sends a video file named madrid.mp4 with a message "HALA MADRID"
2. `!sui` Command

Description: Sends a message and a different football-themed video.
Video: Sends a video file named another_video.mp4 with a message "SUIIIIII"
3. `!vamos` Command

Description: Sends a motivational football message followed by a video.
<br>
Video: Sends a video file named vamos_video.mp4 with a message "Vamos!"
4. `!calma` Command

Description: Sends a football message followed by a video.
<br>
Video: Sends a video file named vamos_video.mp4 with a message "CALMA!"

## Problem it Solves
The bot addresses the need for interactive and engaging football content within a Discord server. It provides a way for football fans to:

1. Stay Engaged: By sending interesting messages and videos related to football.
2. Celebrate Football Moments: Commands like !uno and !sui celebrate moments in football with themed messages and media.
3. Create a Themed Environment: The bot creates a fun atmosphere for football fans with specific commands and media content.

## How it Works
1. Bot Initialization

The bot is initialized using the Serenity library for Discord interactions and the Tokio library for asynchronous operations.
<br>
It loads the Discord token from environment variables and sets up the necessary intents for receiving and sending messages.
2. Command Handling

Commands are defined using Serenity's framework and are grouped under the General group.
<br>
Each command (!uno, !sui, !vamos) is handled by a specific function that sends a series of messages and, in some cases, a video file.
<br>
Commands use the send_message method to deliver messages and the add_file method to send video files.
3. Event Handling

The bot logs messages and events such as the bot’s readiness and incoming messages to the console for debugging purposes.
4. File Management

Video files are specified with relative paths, and the bot ensures these files are accessible for sending within Discord channels.

## Setting Up HALAMADRID Bot
1. First clone the repository <>.Then run `cargo run`
2. Add .env to the root of the folder. Add `DISCORD_TOKEN=`. Go to `https://discord.com/developers/applications` and generate a token which you paste it to the .env file.
3. Invite the Bot to Your Server
Go to the Discord Developer Portal and select your application.
<br>
Under the "OAuth2" tab, select "URL Generator."
<br>
Under "OAuth2 URL Generator," select the "bot" scope and give it appropriate permissions (e.g., "Send Messages").
<br>
Copy the generated URL and use it to invite the bot to your server.

## Step by step Troubleshooting
1. Verify Bot is online
Check Discord Server: Make sure the bot is shown as "online" in your Discord server. If the bot is not online, it means that the bot is not connected correctly, which could be due to a token issue or network connectivity problems.

2. CHeck the console to see if "Received !uno command from..." is printed. // Let's use uno command as an example

3. Enable Intents in Discord Developer Portal: Log in to the Discord Developer Portal, navigate to your bot application, go to the "Bot" tab, and ensure that both MESSAGE CONTENT INTENT and SERVER MEMBERS INTENT are enabled.

4. Check .env File: Ensure the .env file exists in the root of your project and contains the correct DISCORD_TOKEN.

5. Role Permissions: Check that the bot's role in the Discord server has the necessary permissions (Read Messages, Manage Messages, Read Message History, etc.).

## Use Commands
`!uno`
<br>
`!sui`
<br>
`!vamos`
<br>
`!calma`

## Technical Details
Language: Rust
<br>
Libraries: Serenity, Tokio
<br>
Environment: Requires the Discord token to be set in the environment variables.

## Contribution
Contributions are welcome! A football quiz commands is under-development. Coming out soon but not part of the monthly coding challenge submission.

