<?php

namespace App\Models;

use App\Models\Movie;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class Showing extends Model
{
    use HasFactory;

    protected $casts = [
        "show_time" => "datetime",
    ];

    protected $fillable = [
        "isPublic",
        "movie_id",
        "show_time",
        "owner",
    ];

    public function movie()
    {
        return $this->belongsTo(Movie::class);
    }

    public function schedule()
    {
        return $this->belongsTo(Schedule::class);
    }
}
