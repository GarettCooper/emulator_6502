try:
    from BeautifulSoup import BeautifulSoup
except ImportError:
    from bs4 import BeautifulSoup

import requests
import re

html = BeautifulSoup(requests.get("http://www.oxyron.de/html/opcodes02.html").text)
template = "Opcode{{ function: {}, address_mode: {}, cycles: {} }},\t\t//{}"

address_map = {"imm" : "immediate",
               "zp" : "zero_page",
               "zpx" : "zero_page_x",
               "zpy" : "zero_page_y",
               "izx" : "indirect_x",
               "izy" : "indirect_y",
               "abs" : "absolute",
               "abx" : "absolute_x",
               "aby": "absolute_y",
               "ind": "indirect",
               "rel": "relative",
               "" : "implied"
               }

i = 0
for td in filter(lambda tag : not ("font size=\"+1\"" in str(tag)), html.find_all("td")[18:289]):
	opcode = re.search("(?<=>)([A-Z])+", str(td))

	address_mode = re.search("(?<=>)([a-z])+", str(td))
	if address_mode is None: address_mode = ""
	else: address_mode = address_mode.group().lower()

	cycles = re.search("(\d)(?=[*<])", str(td))
	if cycles is None: cycles = 0
	else: cycles = cycles.group()
	print(template.format(opcode.group().lower(), address_map[address_mode], cycles, hex(i)))
	i += 1


