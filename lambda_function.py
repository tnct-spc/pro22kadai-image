import procon_image
import json


def lambda_handler(event, context):
    encoded_img = event['img']
    points = procon_image.get_points(encoded_img)
    return points
