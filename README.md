
# Link Test Live

Together, we can make it the holy grail - 'cause technically, we only need one!

![the_holy_grail](https://github.com/tpmccallum/link-test-live/assets/9831342/ac8dee69-e654-48d2-ab82-eee5b4ffe9c4)

**_Clone this repo, run a node and join the fun._**

# Why Link Test Live (LTL)?

There are many reasons for you to try LTL:
- You get to experience the fastest link checker on the block.
- No infrastructure is needed on your end; it just needs a Linux machine and a network connection.
- You can run your own node and contribute to the global link-checking revolution.
- A stranger smiles every time you check a link.
- It's fun - _apparently_.

# How does LTL work?

Anybody (you) can run the LTL software on your local machine. Here is [a blog post on the matter](https://www.fermyon.com/blog/turbocharging-broken-link-checking).

# Quantity

This is no lazy last-minute link checker (that linearly forces its way through links at build time), LTL is prepared ahead of time and can instantly provide status codes for links whenever you need them.

# Quantity

Your node will check 5 links per minute (300 per hour). If another 99 users just like you run a node, collectively we are checking 30 thousand links per hour. Sp spread the word.

# What’s in It for Me?

You are not only helping others check links; you can submit your own links for checking.

There may be a points system in the future, but for now, don't worry; just join the fun.

# Life Without LTL

Using LTL is the same as checking your own links, except without LTL, you will ~~be lonely~~ need to set up infrastructure. For example, an application and database that can keep track of your URLs as well as an API to fetch status codes etc. With LTL, we do it all for you. You can run your node and forget about it. When you are ready to perform your own link checking for your content, call the LTL API.

# Get Started

Follow the process outlined below to get started.

## Installing Spin

If you haven't already, please go ahead and [install the latest version of Spin](https://developer.fermyon.com/spin/install).

> Upgrading Spin: If you have an older version of Spin, please see the [Spin upgrade page](https://developer.fermyon.com/spin/upgrade).

## Download LTL

Checkout the repository:

```bash
git clone https://github.com/tpmccallum/link-test-live.git
cd link-test-live/link-test-live-cron
```

## Run (Initialize)

Use the following command to run your LTL node:

```bash
## Make sure that you are still in the link-test-live/link-test-live-cron directory
$ spin build --up

Logging component stdio to ".spin/logs/"
Storing default key-value data to ".spin/sqlite_key_value.db"
Test mode disabled ... using cloud ...
Checking for API key, please wait ...
Fetching new batch of links to check ...
Using API Key "bef8596e-DC0A-4356-B4E9-74CD2B4A329E"
Processing: https://developer.fermyon.com/
Processing: https://developer.fermyon.com/spin/v2/upgrade
Processing: https://developer.fermyon.com/spin/v2/quickstart
Processing: https://developer.fermyon.com/spin/v2/
Processing: https://developer.fermyon.com/cloud/quickstart

...

... and so on ...
```

## The API Key

On its maiden voyage, your node will fetch an API key from the Cloud API and automatically save it inside your app. Nothing for you to do here; it's automatic.

## Adding New Links To The System

Let's say, I quite fancy my GitHub home page and this project to be part of the links to be checked. I can add my links (`new_additions`) to the system as new links to check. These links will be forever checked by nodes:

```bash
curl -X POST https://link-test-live-cloud.fermyon.app \
     -H "Content-Type: application/json" \
     -d '{
           "new_additions": [
               "https://github.com/tpmccallum",
               "https://github.com/tpmccallum/link-test-live"

           ]
         }'
```

## Check Links That Are Already In The System

Let's say I want to check those links (`links_to_check`), after someone else's node has kindly checked them for me:

```bash
curl -X POST https://link-test-live-cloud.fermyon.app \
     -H "Content-Type: application/json" \
     -d '{
           "api_key": "bef8596e-DC0A-4356-B4E9-74CD2B4A329E",
           "links_to_check": [
               "https://github.com/tpmccallum",
               "https://github.com/tpmccallum/link-test-live"
           ]
         }'
```

That's about it; [if you want to see the front end in action **click here**](https://link-test-live-cloud.fermyon.app/frontend).
