# Generated by Django 3.0.6 on 2020-05-12 08:36

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [("realm", "0001_initial")]

    operations = [
        migrations.AddField(
            model_name="activity",
            name="url",
            field=models.TextField(default=""),
            preserve_default=False,
        )
    ]