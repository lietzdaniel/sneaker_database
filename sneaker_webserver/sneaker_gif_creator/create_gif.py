
import os
from PIL import Image
from .get_imgs import get_angleimages_from_original_image
import logging
import sys

PROJECT_PATH = os.path.join(".","sneaker_gif_creator")
logging.basicConfig(level=logging.DEBUG, stream=sys.stdout)
logger = logging.getLogger()
def make_gif(link,style_id: str):
 
    style_id = style_id.replace(" ","-")
   
    if not os.path.exists(os.path.join(PROJECT_PATH,style_id)): 
        logger.info(f"Downloading images for {style_id}.")
        get_angleimages_from_original_image(link,style_id)
        logger.info(f"Successfully downloaded images for {style_id}.")
    if not os.path.exists(os.path.join(PROJECT_PATH,style_id,"gif")):
        logger.info(f"Creating gif for {style_id}.")
        image_path = os.path.join(PROJECT_PATH,style_id,"img")
        frames = [Image.open(f"{image_path}/{i}.png") for i in range(1,36)]
    
        frame_one = frames[0]
        os.mkdir(os.path.join(PROJECT_PATH,style_id,"gif"))
        saving_path = os.path.join(PROJECT_PATH,style_id,"gif",f"{style_id}.gif")
        frame_one.save(saving_path, format="GIF", append_images=frames,
                save_all=True, duration=100, loop=0)
        logger.info(f"Successfully created gif for {style_id}.")
            

if __name__ == "__main__":
    make_gif("","DH3227-105")