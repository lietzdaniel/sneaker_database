
from rest_framework.views import APIView
from rest_framework.response import Response
from rest_framework import status
from .models import Shoe
from .serializers import ShoeSerializer

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
            return Response(serializer.data, status=status.HTTP_201_CREATED)

        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)
    


    