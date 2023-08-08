
from django.urls import path
from .views import (
    ShoeListApiView,
)

urlpatterns = [
    path('api', ShoeListApiView.as_view()),
]