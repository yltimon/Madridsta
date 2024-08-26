# HALAMADRID Discord Bot

## Overview

HALAMADRID is a Discord bot tailored for football enthusiasts, offering engaging, football-themed commands to enhance user interaction. The bot sends messages, shares videos, and creates a lively environment for Real Madrid fans and general football lovers. It can celebrate iconic moments, motivate users, and add a fun, football-centric atmosphere to any Discord server.

## Why Use HALAMADRID Bot?

1. Interactive Experience: The bot's commands deliver a unique, interactive experience with themed messages and videos, making conversations more exciting.

2. `Football Focused: Specifically designed for football fans, it brings a shared passion for the sport into everyday chats, making it perfect for sports communities.

3. Easy Integration: Simple to set up and use, HALAMADRID enhances any Discord server with minimal effort.

4. Customization: While initially focused on Real Madrid, it can be adapted to include other football themes and commands.

## Problem it Solves
The bot addresses the need for interactive football content on Discord servers, keeping football fans engaged, celebrating football moments, and creating a themed environment with specific commands and media.

## How it Works
1. Bot Initialization

The bot uses the Serenity library for Discord interactions and Tokio for asynchronous operations. It loads the Discord token from environment variables and sets up intents for messaging.

2. Command Handling

Commands are defined using Serenity's framework and handled by specific functions that send messages and, in some cases, videos.

3. Event Handling

The bot logs readiness and incoming messages to the console for debugging.

4. File Management

Video files are accessed using relative paths, ensuring they're available for sending within Discord channels.

## Video Demo

Watch the video demo on `./demo.mp4`

## Installation and Setup Instructions
1. Clone the Repository

Run git clone `https://github.com/yltimon/Madridsta` then `cd Madrista`.

2. Set Up Environment Variables

Create a `.env` file in the root directory. Add `DISCORD_TOKEN=` followed by your Discord token.

3. Build the Bot

Ensure you have Rust installed on your system. Run the following command to build and start the bot `cargo run`

4. Invite the Bot to Your Server

* Go to the Discord Developer Portal and select your application.
* Under the "OAuth2" tab, use the "OAuth2 URL Generator" to select the "bot" scope and assign permissions (e.g., "Send Messages").
* Copy the generated URL and invite the bot to your server.
* Paste the URL into your browser to invite the bot to your server.

## Troubleshooting
1. Verify Bot is online

Ensure the bot is shown as "online" in your Discord server.

2. Check Console Logs

Confirm the bot logs, such as "Received !uno command from..."

3. Enable Intents

Ensure MESSAGE CONTENT INTENT and SERVER MEMBERS INTENT are enabled in the Discord Developer Portal.

4. Check `.env` File

Verify the `.env` file exists with the correct `DISCORD_TOKEN`.

5. Role Permissions

Ensure the bot's role has necessary permissions in the Discord server.

## Use Commands
`!uno`
<br>
`!sui`
<br>
`!vamos`
<br>
`!calma`

## Conclusion
With ongoing updates and potential for customization, HALAMADRID is the go-to bot for enhancing your football community on Discord.








