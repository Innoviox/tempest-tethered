import csv
from bs4 import BeautifulSoup

soup = BeautifulSoup(open("cards.html").read(), features="lxml")

writer = csv.DictWriter(open("cards.csv", "w"), fieldnames=["Name", "CType", "Cost", "Strength", "Speed", "Effect", "Class", "Kingdom", "Rarity", "Level"])
writer.writeheader()

for i in soup.find_all("tr")[1:]:
    writer.writerow({f: d for f, d in zip(['Name', 'Effect', 'Kingdom', 'CType', 'Class', 'Rarity', 'Cost', 'Strength', 'Speed'], [i.text for i in i.find_all("td")])})
