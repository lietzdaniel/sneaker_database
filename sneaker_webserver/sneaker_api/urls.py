
from django.urls import path
from .views import (
    ShoeListApiView,
    GetShoeGifApiView,
    GetShoePricesApiView
)

urlpatterns = [
    path('api/gif/<str:style_id>/', GetShoeGifApiView.as_view()),
    path('api/json/<str:style_id>/', GetShoePricesApiView.as_view()),
    path('api', ShoeListApiView.as_view()),
]