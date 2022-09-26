from django.db import models
from django.contrib.auth.models import AbstractUser
from .managers import OrgUserManager
from django.utils.translation import gettext_lazy as _


class OrgUser(AbstractUser):
    username = None
    email = models.EmailField(_("email address"), unique=True)

    USERNAME_FIELD = 'email'
    REQUIRED_FIELDS = ['first_name', 'last_name']

    objects = OrgUserManager()

    def __str__(self):
        return self.email
