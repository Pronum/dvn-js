const { Client } = require("discord.js");

const client = new Client({});

client.on('ready', ( ) => {
    console.log('hi');
    console.log(`Bot Tag: ${client.user.tag}`);
});

client.login('sjdnhakjdnajsdas');