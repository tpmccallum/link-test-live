# Link Test Live (LTL)

![link-checker-live (1)](https://github.com/tpmccallum/link-test-live/assets/9831342/77891587-d0f3-4243-adc7-c1670a3c8744)

`More than a broken link checker. Clone this repo, run a node, and join the fun.`

# Why LTL?

There are many reasons for you to try LTL:
- you get to experience the fastest link checker on the block
- no infrastructure is needed on your end, just a Linux machine and a network connection
- you get to run your own node and contribute to the global link-checking revolution
- you get extra points for overachieving
- it's fun - _apparently_

# How does LTL work?

Anybody (you) can run the LTL software on your local machine and help check links for others.

If you configure your LTL node to check just 5 links per minute, you can be responsible for 300 links per hour of freshness.
If another 99 users, just like you, each run a node (that also checks just 5 links per minute), then collectively, you will **all** be responsible for the freshness of 30,000 links per hour.
No current limit exists to the number of users running their own node.
You can go beyond the default batch cadence and number of links per batch to be an overachiever.

# What's in it for me?

When your node performs link checks, your node automatically sends the data to the LTL API. If you fill out the default link checking quota, then your API key will be rewarded with points. These points let you perform lightning-fast broken link checks against the LTL API. If you exceed the quota, you will be given bonus points, entitling you to perform **more** broken link checking against the LTL API.

(You get 1 point for each checked link when your quota is above the default. You get 3 points for each checked link when your quota is above **double** the default quota.)

# Life without LTL

Using LTL is the same as checking your own links yourself, except without LTL, you will ~~be lonely~~ need to set up infrastructure. For example, you must set up an application and database to store URLs and responses and maintain that infrastructure. With LTL, we do it all for you. You can run your node and forget about it. When you are ready to perform your own link checking for your content, you just call the LTL API and get a link-point ratio of 1:1.

# Get Started

Following the process outlined below to get started.

## Installing Spin

If you haven't already, please go ahead and [install the latest version of Spin](https://developer.fermyon.com/spin/install).

> Upgrading Spin: If you have an older version of Spin, please see the [Spin upgrade page](https://developer.fermyon.com/spin/upgrade).

## Download LTL

Checkout the repository:

```bash
https://github.com/tpmccallum/link-test-live.git
cd link-test-live/link-test-live-cron
```

## Get your API key ASAP

## Run

Use the following command to run your LTL node:

```bash
## Make sure that you are still in the link-test-live/link-test-live-cron directory
spin build --up
Thank you - your api key is ltl-0x1234. Please save this key and add to the api_key in your spin.toml file.
Please construct a secure HTTP request 
```

On its maiden voyage, your node will fetch an API key from the Cloud API. **SAVE THIS KEY - IT STORES YOUR POINTS**

Save the key and also add it to your config:

```toml
api_key = { default = "ltl-0x1234" }
```

## Configuration

Open the `spin.toml` file and list any of the domains that you are comfortable with your node visiting. for example you can add `https://developer.fermyon.com` as a trusted site that you are prepared to check links for: 


```toml
allowed_outbound_hosts = ["https://link-test-live-cloud.fermyon.app", "https://developer.fermyon.com"]
```

If you just want to check links for all sites, then use the following wildcard:

```toml
allowed_outbound_hosts = ["https://*"]
```

## Advanced Configuration (Optional)

**You do not need to touch this unless you are an advanced user.**

Advanced users: Configure to suit your needs (if you want to earn bonus points)

### How often you want to be given links to check.

Open the `spin.toml` file and update the `cron_expression`. The default is set to one batch per minute.

```toml
cron_expression = "0 * * * * *"
```

Below is additional information that will help you configure your `cron_expression`.

#### Cron Expression Fields

The `cron_expression` fields are as follows:

```bash
#  ┌──────────── sec (0–59)
#  |    ┌───────────── min (0–59)
#  |    │  ┌───────────── hour (0–23)
#  |    │  │  ┌───────────── day of month (1–31)
#  |    │  │  │  ┌───────────── month (1–12)
#  |    │  │  │  │  ┌───────────── day of week (0–6)
#  |    │  │  │  │  |  ┌─────────────- year
#  |    │  │  │  │  |  │
#  |    │  │  │  │  |  │
  1/30  *  *  *  *  *  * 
```

#### Cron Expression Characters

- The `*` indicates that every value applies; i.e., if `sec` is set to `*`, then every second will trigger execution.
- The `/` indicates increments. For example, if `sec` is set to `0/15`, then starting at `0`, the trigger will be executed every 15 seconds.
- The `,` separates values. For example, if `sec` is set to `2,8`, then the trigger will execute only on the 2nd and 8th seconds of every minute.
- The `-` indicates range, i.e., if the `sec` is set to `5-10`, then the trigger will execute only on the 5th, 6th, 7th, 8th, 9th, and 10th seconds of each minute.
- The `0` indicates no execution. If the `sec` is set to `0`, then the trigger can only execute on higher field values such as `min`, `hour`, etc. With `sec` set to `0` the increments can only be one minute and higher.

### How many links you want to check per batch

**You do not need to touch this unless you are an advanced user.**

The default is preset to `"5"` in the `spin.toml` file, i.e. `links_per_batch = { default = "5" }`.

Advanced users: Set a system variable that lets the LTL API know how many links you will check each time your `cron_expression` is executed. For example:

```bash
export SPIN_VARIABLE_LINKS_PER_BATCH="10"
```

> Where a batch is one cycle of the `cron_expression`, i.e. the default of one-minute batches.

## Advanced

Leave the console running so that the cron can execute.

Running in the background:

```bash
## Make sure that you are still in the link-test-live/link-test-live-cron directory
spin build
spin up > /dev/null 2>&1 &
```

> The above is only recommended for advanced users. Note, you will not receive any logs and you will also need to know how to stop that process from running. _hint: you use a combination of the `jobs` command the `fg` command and  `ctrl` + `c`_

## Checking URLs using LTL Cloud API 

To use the LTL Cloud API create an HTTPS request using the POST verb and include the following JSON format as the body of the request:

```json
{
    "api_key": "ltl-0x1234",
    "urls": [
        "https://developer.fermyon.com/",
        "https://developer.fermyon.com/spin/v2/testing-apps",
        "https://developer.fermyon.com/hub/preview/pattern_long_running_jobs_over_http"
    ]
}
```

The above JSON shows how we use out `api_key`, `urls` (the list of urls that we want the Cloud API to check for us).

Python Example:

```py
# python3 -m venv venv-dir
# source venv-dir/bin/activate
# python3 -m pip install requests

import requests

# URL to which the POST request will be sent
url = 'https://link-test-live-cloud.fermyon.app/'

# List of URLs to be sent as JSON data in the POST request body
data = {
    "api_key": "ltl-0x1234",
    "urls": [
        "https://developer.fermyon.com/",
        "https://developer.fermyon.com/spin/v2/testing-apps",
        "https://developer.fermyon.com/hub/preview/pattern_long_running_jobs_over_http"
    ],
    "allowed_outbound_hosts": ["https://developer.fermyon.com"]
}

# Send the POST request
response = requests.post(url, json=data)
print(response.text)
```

# F.A.Q.

Below are answers to some Frequently Asked Questions:

- What is the default quota?
  The default quota is set to 5 links per batch, and each batch is processed once per minute (300 links per minute).
- How many points do I get to meet the quota?
  You get 1 point for each link you check if your quota is above the default. You get 3 points for each link if your quota is above double the default quota.
- How many points are required to access the LTL API if I want to run a broken link checker on my content?
  Checking one URL will reduce your overall tally of points by 1.
- What quality control is performed?
  The LTL API randomly selects URLs and double-checks the quality of an individual node's work (random sampling).
- What happens if a node sends incorrect information to the LTL API?
  If a node sends incorrect information to the LTL API, no points are given to that node, and the data is not stored.

If you would like to add an F.A.Q please [open an issue](https://github.com/tpmccallum/link-test-live/issues).

