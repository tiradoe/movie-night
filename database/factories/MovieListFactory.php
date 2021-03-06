<?php

namespace Database\Factories;

use App\Models\MovieList;
use Illuminate\Database\Eloquent\Factories\Factory;

class MovieListFactory extends Factory
{
    /**
     * The name of the factory's corresponding model.
     *
     * @var string
     */
    protected $model = MovieList::class;

    /**
     * Define the model's default state.
     *
     * @return array
     */
    public function definition()
    {
        return [
            'id' => $this->faker->unique()->randomNumber(5, true),
            'name' => $this->faker->sentence(),
            'isPublic' => $this->faker->boolean(),
            'owner' => 1,
        ];
    }
}
