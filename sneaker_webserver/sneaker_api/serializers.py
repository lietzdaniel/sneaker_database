from rest_framework import serializers
from .models import Shoe
class ShoeSerializer(serializers.ModelSerializer):
    class Meta:
        model = Shoe
        fields = ["style_id","link","name","type","model","colorway","image","size","release_date","retail_price","last_sold_price","extras","description"]