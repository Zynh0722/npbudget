from django.contrib.auth.forms import UserCreationForm, UserChangeForm

from .models import OrgUser


class OrgUserCreationForm(UserCreationForm):

    class Meta:
        model = OrgUser
        fields = ('email',)


class OrgUserChangeForm(UserChangeForm):

    class Meta:
        model = OrgUser
        fields = ('email',)
