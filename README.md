
# Link Test Live (LTL)

![link-checker-live (1)](https://github.com/tpmccallum/link-test-live/assets/9831342/77891587-d0f3-4243-adc7-c1670a3c8744)

`More than a broken link checker. Clone this repo, run a node, and join the fun.`

# Why LTL?

There are many reasons for you to try LTL:
- You get to experience the fastest link checker on the block.
- No infrastructure is needed on your end, just a Linux machine and a network connection.
- You get to run your own node and contribute to the global link-checking revolution.
- You get extra points for overachieving.
- It's fun - _apparently_.

# How does LTL work?

Anybody (you) can run the LTL software on your local machine and help check links for others.

If you configure your LTL node to check just 5 links per minute, you can be responsible for 300 links per hour of freshness. If another 99 users, just like you, each run a node (that also checks just 5 links per minute), then collectively, you will **all** be responsible for the freshness of 30,000 links per hour. No current limit exists to the number of users running their own node.
You can go beyond the default batch cadence and number of links per batch to be an overachiever.

# What’s in It for Me?

When your node performs link checks, your node automatically sends the data to the LTL API. You are helping check links for yourself and for others.

Long-term idea: If you fill out the default link checking quota, then your API key will be rewarded with points. (The points system is more of a long-term goal/experiment so don't get too caught up in that just now.)

In the future, I might set it up so you get 1 point for each checked link when your quota is above the default. Perhaps you could get 3 points for each checked link when your quota is above **double** the default quota. We will see. For now, I just want everyone to be able to make a secure HTTP request to see the link checker return results. This part will be free for now.

# Life Without LTL

Using LTL is the same as checking your own links yourself, except without LTL, you will ~~be lonely~~ need to set up infrastructure. For example, you must set up an application and database to store URLs and responses and maintain that infrastructure. With LTL, we do it all for you. You can run your node and forget about it. When you are ready to perform your own link checking for your content, you just call the LTL API and get a link-point ratio of 1:1.

# Get Started

Following the process outlined below to get started.

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