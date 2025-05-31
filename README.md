# Termcast
## CLI tool for sending files to Telegram
---

## Getting Started 
### Setup .env 
Termcast utilizes Teloxide for Telegram API interactions. In addition to the default Teloxide varaible you will also need to establish a second value `CHANNEL` which will be your default channel's TelegramID. This value should start with `-100`. 

Example .env
```
TELOXIDE_TOKEN=your_bot_api_key
CHANNEL=your_default_channel_id
```

### Build
Run `cargo run build` to build and get started. 

You will first be prompted to give a path to the you want to send. 

Next, you will be asked if you want to send your file to your default channel. If not, then you can provide a secondary TelegramID to send the file to instead. 

You can enable groupID in Telegram>Settings>Experimental. If you are passing a value straight from this setting you will want to add `-100` to the ID so that the Telegram API handles your request properly. 

Finally, you will be asked to provide a caption for your file. 

---