# python3 -m venv venv-dir 
# source venv-dir/bin/activate
# python3.11 -m pip install --upgrade pip
# python3.11 -m pip install beautifulsoup4

# Add your sitemap URL to the website_sitemap variable which is the first line of the script below

# Run the script with the following command
# python3.11 harvest_links.py
import time
import json
import requests
import xml.dom.minidom as minidom
from bs4 import BeautifulSoup

## website_sitemap variable
website_sitemap = requests.get('https://developer.fermyon.com/sitemap.xml', allow_redirects=True).text
parsed_website_sitemap_document = minidom.parseString(website_sitemap)
website_sitemap_loc_elements = parsed_website_sitemap_document.getElementsByTagName('loc')
website_page_urls = []

for website_sitemap_loc_element in website_sitemap_loc_elements:
    website_page_urls.append(website_sitemap_loc_element.toxml().removesuffix("</loc>").removeprefix("<loc>"))

print("Number of page to process is {}\n First page to process is {} and the last page to process is {}".format(len(website_page_urls), website_page_urls[0], website_page_urls[len(website_page_urls) - 1]))

page_url_dict = {}
page_int = 1
for one_page in website_page_urls:
    print(f"Processing {one_page}")
    page_url_dict = {}
    temp_list = []
    response = requests.get(one_page)
    if response.status_code == 200:
        if one_page.startswith("http"):
            #if page_int == 2:
            #    continue
            soup = BeautifulSoup(response.content, 'html.parser')
            links = soup.find_all('a')
            urls = [link.get('href') for link in links]
            urls = [url for url in urls if url is not None and (url.startswith('http') or url.startswith('https'))]
            urls = list(set(urls))
            current_time_epoch = int(time.time())
            sql_insert_statement = "INSERT OR IGNORE INTO URLTable (single_url, last_checked_epoch, response_code, status_code) VALUES\n"
            values = [
                f"('{url}', {current_pointer_epoch}, NULL, 'pending')"
                for url in urls
                for current_pointer_epoch in (current_time_epoch,)  # Fixed current time for all URLs
            ]
            sql_insert_statement += ",\n".join(values) + ";"
            filename = 'insert_urls_' + str(page_int) + '.sql'
            page_int += 1
            with open(filename, 'w') as file:
                file.write(sql_insert_statement)

""" 
This script will generate a SQL file with the following content:


Loading this SQL file into the database is done in the following way:

For the local app `spin up --sqlite @insert_urls.sql`
For the Cloud app `spin cloud sqlite execute -d mydb @insert_urls.sql`

If fragmenting the files, into a SQLite page per file, then here is a bash script to load all the SQL files into the Cloud API database:

#!/bin/bash

# Loop through each file that starts with 'insert_urls_' and ends with '.sql'
for sql_file in insert_urls_*.sql
do
  echo "Executing ${sql_file}..."
  # Execute the command with the current SQL file
  spin cloud sqlite execute -d mydb @"${sql_file}"
  
  # Check if the command was successful
  if [ $? -eq 0 ]; then
    echo "${sql_file} executed successfully."
  else
    echo "Failed to execute ${sql_file}."
  fi

  # Pause for 2 seconds
  sleep 2
done


Here is a bash script to insert the sql locally:

#!/bin/bash

# Loop through each file that starts with 'insert_urls_' and ends with '.sql'
for sql_file in insert_urls_*.sql
do
  echo "Executing ${sql_file}..."
  # Execute the command with the current SQL file
  spin cloud sqlite execute -d mydb @"${sql_file}"
  
  # Check if the command was successful
  if [ $? -eq 0 ]; then
    echo "${sql_file} executed successfully."
  else
    echo "Failed to execute ${sql_file}."
  fi

  # Pause for 2 seconds
  sleep 2
done

You can then check the database out using the local CLI

sqlite3 .spin/sqlite_db.db
sqlite> .tables


"""


