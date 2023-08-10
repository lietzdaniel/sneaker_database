
from rest_framework.views import APIView
from rest_framework.response import Response
from rest_framework import status
from .models import Shoe
from django.http import HttpResponse
from .serializers import ShoeSerializer
from sneaker_info_getter.create_gif import make_gif
from sneaker_info_getter.get_prices import create_json
from django.http import JsonResponse
from multiprocessing import Process
import os
import asyncio
import json

INFO_PATH = os.path.join(".","sneaker_info_getter")
class ShoeListApiView(APIView):
    def get(self, request):
        try:
            shoes = Shoe.objects.all()
            serializer = ShoeSerializer(shoes, many=True)
            
            return Response(serializer.data, status=status.HTTP_200_OK)
        except Exception as e:
            return Response({"error": str(e)}, status=status.HTTP_500_INTERNAL_SERVER_ERROR)
    def post(self, request, *args, **kwargs):
        data = {
            'style_id': request.data.get('style_id'), 
            'link': request.data.get('link'), 
            'type': request.data.get('type'), 
            'name': request.data.get('name'), 
            'shoe_type': request.data.get('shoe_type'), 
            'model': request.data.get('model'), 
            'colorway': request.data.get('colorway'), 
            'image': request.data.get('image'), 
            'size': request.data.get('size'), 
            'release_date': request.data.get('release_date'), 
            'retail_price': request.data.get('retail_price'), 
            'last_sold_price': request.data.get('last_sold_price'), 
            'extras': request.data.get('extras'), 
            'description': request.data.get('description'), 

        }
        serializer = ShoeSerializer(data=data)
        if serializer.is_valid():
            serializer.save()
            create_json(data['link'], data['style_id'])
            
            gif_process = Process(target=make_gif,args=(data["image"],data["style_id"]))
            gif_process.start()
            return Response(serializer.data, status=status.HTTP_201_CREATED)

        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)
    

class GetShoeGifApiView(APIView):
    def get(self, request, style_id):
        try:
            shoe = Shoe.objects.get(style_id=style_id)
            style_id = style_id.replace(" ","-")
            style_id = style_id.replace("/","-")
            gif_path = os.path.join(INFO_PATH,style_id,"gif")
            if not os.path.exists(gif_path):
                return Response({'error': 'Shoe not found'}, status=404)
            gif_file_path = os.path.join(gif_path,f"{style_id}.gif")
            with open(gif_file_path, 'rb') as gif_file:
                response = HttpResponse(gif_file.read(), content_type='image/gif')
                response['Content-Type'] = 'image/gif'
                response['Content-Disposition'] = f'inline; filename="{style_id}.gif"'


                return response
        except Shoe.DoesNotExist:
            return Response({'error': 'Shoe not found'}, status=404)
        
class GetShoePricesApiView(APIView):
    def get(self, request, style_id):
        try:
            shoe = Shoe.objects.get(style_id=style_id)
        
            json_path = os.path.join(INFO_PATH,style_id,"prices")
            if not os.path.exists(json_path):
                return Response({'error': 'Prices not found'}, status=404)
            json_file_path = os.path.join(json_path,f"{style_id}.json")
            shoe_json = open(json_file_path)
            shoe_prices = json.load(shoe_json)

            return  JsonResponse(shoe_prices)
        except Shoe.DoesNotExist:
            return Response({'error': 'Shoe not found'}, status=404)
