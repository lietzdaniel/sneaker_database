
from django.urls import path
from .views import (
    ShoeListApiView,
    GetShoeGifApiView
)

urlpatterns = [
    path('api/gif/<str:style_id>/', GetShoeGifApiView.as_view()),
    path('api', ShoeListApiView.as_view()),
]