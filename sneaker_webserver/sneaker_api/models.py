from django.db import models

class Shoe(models.Model):
    style_id = models.TextField()
    link = models.TextField()
    name = models.TextField()
    type = models.TextField()
    model = models.TextField()
    colorway = models.TextField()
    image = models.TextField()
    size = models.TextField()
    release_date = models.DateTimeField()
    retail_price = models.TextField()
    last_sold_price = models.TextField(blank=True, null=True)
    extras = models.TextField(blank=True, null=True)
    description = models.TextField()
   
    def __str__(self):
        return self.name