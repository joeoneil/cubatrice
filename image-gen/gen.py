#!/usr/bin/env python

from PIL import Image, ImageFont, ImageDraw
import json
import os

root_img_dir = "../GIMP"
# TODO: Make these paths not absolute (BEFORE COMMIT)
root_data_dir = f"{os.environ['HOME']}/.local/share/Cubatrice/data"
out_img_dir = f"{os.environ['HOME']}/.local/share/Cubatrice/img"
card_dir = f"{root_img_dir}/Cards"
cube_dir = f"{root_img_dir}/Cubes"
tech_dir = f"{root_data_dir}/techConverters"

card_size = (2800, 2000)
name_bbox = (500, 50, 2300, 250)
name_bbox_upg = (200, 50, 2600, 250)
cost_bbox = (1000, 275, 1800, 425)
conv_bbox = (375, 550, 2425, 1500)

arrow_small = Image.open(f"{root_img_dir}/Etc/arrow_small.png").resize((125, 100))

def tech_names(id):
    match id:
        case 1:
            return "Quantum Computers"
        case 101:
            return "Nondeterministic Polynomial Collapse"
        case 2:
            return "Universal Translator"
        case 102:
            return "Universal Empathetic Communication"
        case 3:
            return "Nanotechnology"
        case 103:
            return "Nanofabricators"
        case 4:
            return "Atomic Transmutations"
        case 104:
            return "Pseudomaterials"
        case 5:
            return "Genetic Engineering"
        case 105:
            return "Genetic Resynthesis"
        case 6:
            return "Clinical Immortality"
        case 106:
            return "Practical Immortality"
        case 7:
            return "Ubiquitous Cultural Repository"
        case 107:
            return "Upgraded Ubiquitous (idfk)"
        
        case 8:
            return "Hyperspace Mining"
        case 108:
            return "Hyperspace Settlements"
        case 9:
            return "Cross Species Ethical Equality"
        case 109:
            return "Upgraded Cross (idfk)"
        case 10:
            return "Antimatter Power"
        case 110:
            return "Antimatter Compounds"
        case 11:
            return "Achronal Analysis"
        case 111:
            return "Time Viewers"
        case 12:
            return "Singulary Control"
        case 112:
            return "Wormhole Grid"
        case 13:
            return "Interspecies Medical Exchange"
        case 113:
            return "Panbiologic Medicine"
        case 14:
            return "Organic Construction"
        case 114:
            return "Upgraded Organic (idfk)"

        case 15:
            return "Megastructures"
        case 115:
            return "Dyson Swarms"
        case 16:
            return "Social Exodus"
        case 116:
            return "Galactic Colonization" # Might be wrong
        case 17:
            return "Matter Generation"
        case 117:
            return "Upgraded Matter Gen (idfk)"
        case 18:
            return "Galactic Telecom Control"
        case 118:
            return "Upgraded Galactic (idfk)"
        case 19:
            return "Poly Species Corporations"
        case 119:
            return "Upgraded Poly (idfk)"
        case 20:
            return "Xeno Cultural Exchange"
        case 120:
            return "Upgrded Xeno (idfk)"
        case 21:
            return "Temporal Dilation"
        case 121:
            return "Stasis Field"


def load_cards():
    cards = {}
    races = [
        "caylion",
        "eniet",
        "faderan",
        "imdril",
        "kit",
        "kjas",
        "unity",
        "yengii",
        "zeth",
    ]
    for r in races:
        cards[f"{r}"] = Image.open(f"{card_dir}/{r}_unupgraded.png")
        cards[f"{r}_u"] = Image.open(f"{card_dir}/{r}_upgraded.png")
    return cards
    

def load_cubes():
    cubes = {}
    cube_files = [
        "biotech",
        "culture",
        "food",
        "industry",
        "information",
        "power",
        "ultratech",
    ]
    for c in cube_files:
        cubes[f"{c}"] = Image.open(f"{cube_dir}/{c}_normal.png")
        cubes[f"{c}_d"] = Image.open(f"{cube_dir}/{c}_donation.png")
    return cubes


def cost(cubes):
    sum = 0
    for c in cubes:
        match list(c.items())[0]:
            case ("Cubes" | "DonationCubes", ("Biotech" | "Power" | "Information", qty)):
                sum += 3 * qty
            case ("Cubes" | "DonationCubes", ("Culture" | "Food" | "Industry" | "Ship", qty)):
                sum += 2 * qty
            case ("Cubes" | "DonationCubes", ("Ultratech", qty)):
                sum += 6 * qty
            case ("VictoryPoint" | "DonationVictoryPoint", qty):
                sum += 6 * qty
            case _:
                print(f"Bad input {c}")
    if sum % 2 == 1:
        return f"{sum // 2}½"
    else:
        return f"{sum // 2}"


def render(im, font, name, upg, conv):
    d = ImageDraw.Draw(im)
    font_size = 150
    f = ImageFont.truetype(font, size=font_size)
    # Render Title
    while (f.getlength(name) > name_bbox[2] - name_bbox[0] and not upg) \
        or (f.getlength(name) > name_bbox_upg[2] - name_bbox_upg[0] and upg):
        print(f"Overlong string {name} at size {font_size}. Shrinking")
        font_size -= 10
        f = ImageFont.truetype(font, size=font_size)
    d.text(((name_bbox[2] + name_bbox[0]) / 2, name_bbox[3] - 100), name, anchor="mm", font=f)
    
    # Render input / output costs
    in_cost = cost(conv["input"])
    out_cost = cost(conv["output"])
    font_size = 125
    f = ImageFont.truetype(font, size=font_size)
    total_width = f.getlength(in_cost) + f.getlength(out_cost) + arrow_small.width + 50
    left = ((cost_bbox[2] + cost_bbox[0]) / 2) - (total_width / 2)
    d.text((left, cost_bbox[3]), in_cost, anchor="lb", font=f)
    d.bitmap((left + f.getlength(in_cost) + 25, cost_bbox[3] - arrow_small.height), arrow_small)
    d.text((left + arrow_small.width + 50 + f.getlength(in_cost), cost_bbox[3]), out_cost, anchor="lb", font=f)



    return im


def load_techs():
    techs = {}
    races = [
        "Caylion",
        "Eni Et",
        "Faderan",
        "Imdril",
        "Kit",
        "Kjas",
        "Yengii",
        "Zeth"
    ]
    for r in races:
        convs = json.loads(open(f"{tech_dir}/{r}.json", "r").read())
        r = "".join(r.split(" ")).lower()
        techs[r] = {}
        for c in convs:
            techs[r][c["id"]] = c
    return techs


def main():
    cards = load_cards()
    cubes = load_cubes()
    techs = load_techs()
    for (race, card) in cards.items():
        upgraded = False
        if race[-2:] == '_u':
            race = race[:-2]
            upgraded = True
        if race == 'unity':
            continue
        for (id, tech) in techs[race].items():
            if upgraded ^ (id > 100):
                continue
            print(f"Creating {race} {tech_names(id)} ... ", end="")
            out = render(card.copy(), "Evogria", tech_names(id), upgraded, tech)
            print("Done")
            out.save(f"{out_img_dir}/{race}_{tech_names(id)}.png")

if __name__ == '__main__':
    main()
