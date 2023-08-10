import requests
import os
import shutil
import logging
from PIL import Image
PROJECT_PATH = os.path.join(".","sneaker_gif_creator")
def get_angleimages_from_original_image(original_image_link,style_id):
    os.makedirs(os.path.join(PROJECT_PATH,style_id))
    logging.info(msg = f"Created style_id folder under {os.path.join(PROJECT_PATH,style_id)}")
    os.makedirs(os.path.join(PROJECT_PATH,style_id,"img"))
    link_template = original_image_link.rsplit("/",1)[0]

    for i in range(36):
        if i < 10:
            response = requests.get(f"{link_template}/img0{i}.jpg", stream=True)
            if response.status_code == 200:
                logging.info(msg=f"Successfully accessed the Website, saving image {i}")
                with open(os.path.join(PROJECT_PATH,style_id,"img",f"{i}.png"), 'wb') as file:
                    response.raw.decode_content = True
                    shutil.copyfileobj(response.raw, file)  
                    logging.info(msg=f"Successfully saved image {i}")
           
             
                image = Image.open(os.path.join(PROJECT_PATH,style_id,"img",f"{i}.png"))
                max_width = 1800
                width_percent = (max_width/float(image.size[0])) 
                new_height = int((float(image.size[1])*float(width_percent))) # Keep Aspect Ratio
                resized_image = image.resize((max_width,new_height),Image.Resampling.LANCZOS)
                resized_image.save(os.path.join(PROJECT_PATH,style_id,"img",f"{i}.png"))
        else:
            response = requests.get(f"{link_template}/img{i}.jpg", stream=True)
            if response.status_code == 200:
                logging.info(msg=f"Successfully accessed the Website, saving image {i}")
                with open(os.path.join(PROJECT_PATH,style_id,"img",f"{i}.png"), 'wb') as file:
                    response.raw.decode_content = True
                    shutil.copyfileobj(response.raw, file)  
                    logging.info(msg=f"Successfully saved image {i}")
                image = Image.open(os.path.join(PROJECT_PATH,style_id,"img",f"{i}.png"))
                max_width = 1800
                width_percent = (max_width/float(image.size[0])) 
                new_height = int((float(image.size[1])*float(width_percent))) # Keep Aspect Ratio
                resized_image = image.resize((max_width,new_height),Image.Resampling.LANCZOS)
                resized_image.save(os.path.join(PROJECT_PATH,style_id,"img",f"{i}.png"))



def test(link,image):
    get_angleimages_from_original_image(link,image)

if __name__ == "__main__":
    test("https://images.stockx.com/360/Air-Jordan-1-High-OG-SP-fragment-design-x-Travis-Scott/Images/Air-Jordan-1-High-OG-SP-fragment-design-x-Travis-Scott/Lv2/img01.jpg?fm=avif&auto=compress&w=576&dpr=2&updated_at=1635344578&h=384&q=75","DH3227-105")