(This is currently under **heavy** construction and will be operational by 31st May 2024; if you try it out please submit issues if you are not receiving the desired results)

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

# What’s in It for Me?

When your node performs link checks, your node automatically sends the data to the LTL API. If you fill out the default link checking quota, then your API key will be rewarded with points. These points let you perform lightning-fast broken link checks against the LTL API. If you exceed the quota, you will be given bonus points, entitling you to perform **more** broken link checking against the LTL API.

(You get 1 point for each checked link when your quota is above the default. You get 3 points for each checked link when your quota is above **double** the default quota.)

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
https://github.com/tpmccallum/link-test-live.git
cd link-test-live/link-test-live-cron
```

## Run (Initialize)

Use the following command to run your LTL node:

```bash
## Make sure that you are still in the link-test-live/link-test-live-cron directory
spin build --up
```

## The API Key

On its maiden voyage, your node will fetch an API key from the Cloud API and automatically save it inside your app.

## Configuration (Optional)

### Restrict Allowed Outbound Hosts (Optional)

#### Step 1 (Optional)

The application's configuration is set to allow outbound requests to any host:

```toml
allowed_outbound_hosts = ["https://*"]
```

If you want to change this, open the `spin.toml` file and list any domains you are comfortable visiting with your node. For example, you can add `https://developer.fermyon.com` as a trusted site that you are prepared to check links for: 

```toml
allowed_outbound_hosts = ["https://link-test-live-cloud.fermyon.app", "https://developer.fermyon.com"]
```

Wasm’s sand-boxed execution means no technical or security side-effects will occur if you check links from unknown hosts. Changing this configuration is just a personal preference. Just keep in mind that broken link checkers are not restricted in this way and should ideally be allowed to check links from any host i.e. the default setting.

#### Step 2 (Optional)

The above configuration allows your node to make outbound requests. The following `export` command lets your node tell the LTL Cloud API which hosts you have cherry-picked to check:

```bash
export SPIN_VARIABLE_HOST_WHITELIST='["https://*"]'
```

> Hint: Just paste straight from the `allowed_outbound_hosts` and make sure you use the `'` on each end, as shown above.

Similar to Step 1, if you want to just check all links, then use the following `export` command:

```bash
export SPIN_VARIABLE_HOST_WHITELIST='["https://*"]'
```

You can check the following using the `echo` command like this:

```bash
# All hosts
export SPIN_VARIABLE_HOST_WHITELIST='["https://*"]'
echo $SPIN_VARIABLE_HOST_WHITELIST
["https://*"]
```

```bash
# Restriced hosts
export SPIN_VARIABLE_HOST_WHITELIST='["https://link-test-live-cloud.fermyon.app", "https://developer.fermyon.com"]'
echo $SPIN_VARIABLE_HOST_WHITELIST
["https://link-test-live-cloud.fermyon.app", "https://developer.fermyon.com"]
```

### Advanced Configuration (Optional)

The configuration above is fine. But if you want to turn it up to 11, please feel free to stop your node, try the following instructions, and restart it.

**You do not need to touch this unless you are an advanced user.**

Advanced users: Configure to suit your needs (if you want to earn bonus points)

### How Often Do You Want to Be Given Links to Check?

Open the `spin.toml` file and update the `cron_expression`. The default is set to one batch per minute.

```toml
cron_expression = "0 * * * * *"
```

Below is additional information that will help you configure your `cron_expression`.

#### Cron Expression Fields

The `cron_expression` fields are as follows:

```bash
#  ┌──────────── sec (0–59)
#  |  ┌───────────── min (0–59)
#  |  │  ┌───────────── hour (0–23)
#  |  │  │  ┌───────────── day of month (1–31)
#  |  │  │  │  ┌───────────── month (1–12)
#  |  │  │  │  │  ┌───────────── day of week (0–6)
#  |  │  │  │  │  |  ┌─────────────- year
#  |  │  │  │  │  |  │
#  |  │  │  │  │  |  │
   0  *  *  *  *  *  * 
```

#### Cron Expression Characters

- The `*` indicates that every value applies; i.e., if `sec` is set to `*`, then every second will trigger execution.
- The `/` indicates increments. For example, if `sec` is set to `0/15`, then starting at `0`, the trigger will be executed every 15 seconds.
- The `,` separates values. For example, if `sec` is set to `2,8`, then the trigger will execute only on the 2nd and 8th seconds of every minute.
- The `-` indicates range, i.e., if the `sec` is set to `5-10`, then the trigger will execute only on the 5th, 6th, 7th, 8th, 9th, and 10th seconds of each minute.
- The `0` indicates no execution. If the `sec` is set to `0`, then the trigger can only execute on higher field values such as `min`, `hour`, etc. With `sec` set to `0` the increments can only be one minute and higher.

### How Many Links Do You Want to Check per Batch

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

## Checking URLs Using LTL Cloud API 

To use the LTL Cloud API create an HTTPS request using the POST verb and include the following JSON format as the body of the request:

```bash
curl -X POST https://link-test-live-cloud.fermyon.app \
     -H "Content-Type: application/json" \
     -d '{
           "api_key": "abcd1234-1234-1234-1234-ABCD1234ABCD1",
           "urls": [
               "https://developer.fermyon.com/",
               "https://developer.fermyon.com/spin/v2/testing-apps",
               "https://developer.fermyon.com/hub/preview/pattern_long_running_jobs_over_http"
           ]
         }'
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

# Limitations

- You need to use source code to delete the API key (because the API key is automatically stored in ap's kv). This is not really a problem if you just run the tool as intended and never want to use a different API key.
- The `allowed_outbound_hosts` can't be seen by the app. So, we need you to paste the `allowed_outbound_hosts` content into the `allowed_outbound_hosts` variable also. This is being worked on in an [improvement proposal](https://github.com/fermyon/spin/pull/2454/files#r1616500168) and no timeline is set just yet. For now, please just make sure that URLs are set in both places.

If you would like to add an F.A.Q please [open an issue](https://github.com/tpmccallum/link-test-live/issues).

# Testing

Use `mode_of_operation` use used for local testing:

```bash
export SPIN_VARIABLE_MODE_OF_OPERATION="local"
echo $SPIN_VARIABLE_MODE_OF_OPERATION
local
```

Versus the default of:

```toml
[variables]
...
mode_of_operation = { default = "cloud" }

[component.link-test-live-cron.variables]
...
mode = "{{ mode_of_operation }}"
```

To change it back use:

```bash
export SPIN_VARIABLE_MODE_OF_OPERATION="cloud"
echo $SPIN_VARIABLE_MODE_OF_OPERATION
cloud
```