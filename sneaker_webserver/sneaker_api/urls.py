from django.conf.urls import url
from django.urls import path, include
from .views import (
    ShoeListApiView,
)

urlpatterns = [
    path('api', ShoeListApiView.as_view()),
]